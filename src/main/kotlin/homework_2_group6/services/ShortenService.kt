package homework_2_group6.services

import homework_2_group6.models.Link
import homework_2_group6.models.linkStorage
import io.ktor.application.*
import io.ktor.http.*
import io.ktor.response.*
import kotlin.random.Random


class ShortenService {

    fun getShortUrl(fullUrl: String): String {
        var shortUrl = genShortUrl(6)

        while (linkStorage.containsKey(shortUrl)) {
            shortUrl = genShortUrl(6)
        }

        linkStorage[shortUrl] = Link(fullUrl, shortUrl)
        return shortUrl
    }

    fun getFullUrl(shortUrl: String): Link? {
        return if (linkStorage.containsKey(shortUrl)) {
            val url = linkStorage[shortUrl]
            url!!
        } else {
            null
        }
    }

    fun getAll(): List<Link> {
        return linkStorage.values.toList()
    }

    private fun genShortUrl(keyLength: Int): String {
        val chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".toCharArray()
        val sb = StringBuilder(keyLength)

        for (i in 0..keyLength) {
            sb.append(chars[Random.nextInt(chars.size)])
        }

        return sb.toString()
    }

}