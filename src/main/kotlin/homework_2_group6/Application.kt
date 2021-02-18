package homework_2_group6

import homework_2_group6.routes.linkRoutes
import io.ktor.application.*
import io.ktor.features.*
import io.ktor.gson.*
import io.ktor.response.*
import io.ktor.routing.*
import io.ktor.server.engine.*
import io.ktor.server.netty.*

fun main(args: Array<String>) {
    val port = args.getOrNull(0)?.toIntOrNull()?: 8080

    val server = embeddedServer(Netty, port = port) {
        install(ContentNegotiation) {
            gson {
                setPrettyPrinting()
            }
        }
        routing {
            get("/ping") {
                call.respondText("Pong")
            }
            this.linkRoutes()
        }
    }
    server.start(wait = true)
}
