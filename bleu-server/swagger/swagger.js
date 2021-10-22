const fs = require("fs")
const express = require("express")
const pathToSwaggerUi = require("swagger-ui-dist").absolutePath()
const path = require("path")

require('dotenv').config({ path: path.join(__dirname, "../.env") });

const swaggerResource = `http://${process.env.SERVER_HOST}:${process.env.SERVER_PORT}/api/spec`;
const swaggerPort = process.env.SWAGGER_PORT;

const indexContent = fs.readFileSync(`${pathToSwaggerUi}/index.html`).toString()
    .replace("https://petstore.swagger.io/v2/swagger.json", swaggerResource)

const app = express()

app.get("/", (req, res) => res.send(indexContent))
app.get("/index.html", (req, res) => res.send(indexContent))
app.use(express.static(pathToSwaggerUi))

app.listen(swaggerPort)