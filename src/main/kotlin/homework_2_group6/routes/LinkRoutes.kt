package homework_2_group6.routes

import homework_2_group6.models.JsonMessage
import homework_2_group6.models.LinkResponse
import homework_2_group6.services.ShortenService
import io.ktor.application.*
import io.ktor.http.*
import io.ktor.request.*
import io.ktor.response.*
import io.ktor.routing.*
import io.ktor.util.*

fun Routing.linkRoutes() {

    val linkService = ShortenService()

    get("/l") {
        val list = linkService.getAll()
        call.respond(list)
    }

    get("/l/{short_url}") {
        val shortUrlParam: String = call.parameters.getOrFail<String>("short_url")
        val url = linkService.getFullUrl(shortUrlParam)

        if (url == null) {
            call.respond(HttpStatusCode.NotFound, JsonMessage("Link Not Found"))
        } else {
            call.application.environment.log.info("Full URL : $url")

            val fullUrl = url.fullUrl

            val isHaveProtocol = fullUrl.startsWith("http://")
                    || fullUrl.startsWith("https://")

            var result = fullUrl
            if (!isHaveProtocol) {
                result = "http://${fullUrl}"
            }

            call.respondRedirect(result, true)
        }
    }

    post("/link") {
        val linkReq = call.receive<LinkResponse>()
        val shortUrl = linkService.getShortUrl(linkReq.url)

        call.application.environment.log.info("Short URL : $shortUrl")

        val baseUrl = "http://sh.a6.tnpl.me"
        call.respond(HttpStatusCode.OK, LinkResponse("${baseUrl}/$shortUrl"))
    }

}

