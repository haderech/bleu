const fs = require("fs")
const express = require("express")
const pathToSwaggerUi = require("swagger-ui-dist").absolutePath()

const indexContent = fs.readFileSync(`${pathToSwaggerUi}/index.html`).toString()
    .replace("https://petstore.swagger.io/v2/swagger.json", "http://localhost:8080/api/spec")

const app = express()

app.get("/", (req, res) => res.send(indexContent))
app.get("/index.html", (req, res) => res.send(indexContent))
app.use(express.static(pathToSwaggerUi))

app.listen(3000)