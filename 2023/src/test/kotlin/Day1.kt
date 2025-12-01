import kotlin.test.Test
import kotlin.test.assertEquals

private val SAMPLE_INPUT1 = """
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
        """.trimIndent()

private val SAMPLE_INPUT2 = """
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
        """.trimIndent()

private val DIGITS = setOf(
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
)

private fun String.firstDigit() = findAnyOf(DIGITS)?.let { (_, digit) ->
    DIGITS.indexOf(digit) % 10
}

private fun String.lastDigit() = findLastAnyOf(DIGITS)?.let { (_, digit) ->
    DIGITS.indexOf(digit) % 10
}

private fun part1(input: String): Int {
    return input.lines()
        .filterNot { it.isBlank() }
        .sumOf { line ->
            val digits = line.filter { it.isDigit() }

            val firstDigit = digits.first().digitToInt()
            val lastDigit = digits.last().digitToInt()

            firstDigit * 10 + lastDigit
        }
}

private fun part2(input: String): Int {
    return input.lines()
        .filterNot { it.isBlank() }
        .sumOf { line ->
            val firstDigit = line.firstDigit()!!
            val lastDigit = line.lastDigit()!!

            firstDigit * 10 + lastDigit
        }
}

class Day1 : BaseDay() {
    @Test
    fun part1() {
        assertEquals(142, part1(SAMPLE_INPUT1))
        assertEquals(55017, part1(INPUT))
    }

    @Test
    fun part2() {
        assertEquals(281, part2(SAMPLE_INPUT2))
        assertEquals(53539, part2(INPUT))
    }
}
