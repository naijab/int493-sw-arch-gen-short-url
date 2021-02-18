package homework_2_group6.models

data class Link(val fullUrl: String, val shortUrl: String)

data class LinkResponse(val url: String)

val linkStorage = HashMap<String, Link>()