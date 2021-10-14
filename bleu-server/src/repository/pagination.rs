use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::query_builder::*;
use diesel::query_dsl::methods::LoadQuery;
use diesel::sql_types::{BigInt, HasSqlType};
use serde::Serialize;

const DEFAULT_PAGE_SIZE: i64 = 10;

#[derive(QueryId)]
pub struct Paginated<T> {
    query: T,
    page: i64,
    page_size: i64,
}

pub trait Paginate: Sized {
    fn paginate(self, page: i64) -> Paginated<Self>;
}

impl<T> Paginate for T {
    fn paginate(self, page: i64) -> Paginated<Self> {
        Paginated {
            query: self,
            page_size: DEFAULT_PAGE_SIZE,
            page,
        }
    }
}

impl<T> QueryFragment<Pg> for Paginated<T>
    where
        T: QueryFragment<Pg>,
{
    fn walk_ast(&self, mut out: AstPass<Pg>) -> QueryResult<()> {
        out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
        self.query.walk_ast(out.reborrow())?;
        out.push_sql(") t LIMIT ");
        out.push_bind_param::<BigInt, _>(&self.page_size)?;
        out.push_sql(" OFFSET ");
        let offset = (self.page - 1) * self.page_size;
        out.push_bind_param::<BigInt, _>(&offset)?;
        Ok(())
    }
}

impl<T: Query> Query for Paginated<T> {
    type SqlType = (T::SqlType, BigInt);
}

impl<T> RunQueryDsl<PgConnection> for Paginated<T> {}

impl<T> Paginated<T> {
    pub fn page_size(self, page_size: i64) -> Self {
        Paginated { page_size, ..self }
    }

    pub fn load_and_count_pages<U:Serialize>(self, conn: &PgConnection) -> QueryResult<PaginatedRecord<U>>
        where
            Self: LoadQuery<PgConnection, (U, i64)>,
    {
        let page = self.page;
        let page_size = self.page_size;
        let results = self.load::<(U, i64)>(conn)?;
        let total = results.get(0).map(|x| x.1).unwrap_or(0);
        let records = results.into_iter().map(|x| x.0).collect();
        let total_pages = (total as f64 / page_size as f64).ceil() as i64;
        Ok(PaginatedRecord::from(page, page_size, total_pages, total, records))
    }
}

pub trait LoadPaginated<U: Serialize>: Query + QueryId + QueryFragment<Pg> + LoadQuery<PgConnection, U> {
    fn load_with_pagination(self, conn: &PgConnection, page: i64, page_size: i64) -> QueryResult<PaginatedRecord<U>>;
}

impl<T, U> LoadPaginated<U> for T
    where
        Self: Query + QueryId + QueryFragment<Pg> + LoadQuery<PgConnection, U>,
        U: Queryable<Self::SqlType, Pg> + Serialize,
        Pg: HasSqlType<Self::SqlType>,
{
    fn load_with_pagination(self, conn: &PgConnection, page: i64, page_size: i64) -> QueryResult<PaginatedRecord<U>> {
        self.paginate(page).page_size(page_size).load_and_count_pages::<U>(conn)
    }
}

#[derive(Serialize)]
pub struct PaginatedRecord<U: Serialize> {
    page: i64,
    count: i64,
    total_page: i64,
    total_count: i64,
    records: Vec<U>,
}

impl<U: Serialize> PaginatedRecord<U> {
    pub fn from(page: i64, count: i64, total_page: i64, total_count: i64, records: Vec<U>) -> Self {
        PaginatedRecord {
            page,
            count,
            total_page,
            total_count,
            records,
        }
    }
}