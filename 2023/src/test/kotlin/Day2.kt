import kotlin.math.max
import kotlin.test.Test
import kotlin.test.assertEquals

private val SAMPLE_INPUT = """
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        """.trimIndent()

private val BAG_CONTENTS = mapOf(
    "red" to 12,
    "green" to 13,
    "blue" to 14
)


data class Game(val id: Int, val turns: List<Turn>) {
    val isValid = turns.all { it.isValid }

    val requiredCubes = mutableMapOf<String, Int>()
        .also { cubes ->
            for (turn in turns) {
                for ((color, count) in turn.cubes) {
                    cubes[color] = max(cubes.getOrDefault(color, 0), count)
                }
            }
        }
        .toMap()

    val power = requiredCubes.values.reduce { acc, i -> acc * i }

    companion object {
        fun parse(line: String): Game {
            val match = GAME_REGEX.matchEntire(line)!!

            val id = match.groups[1]!!.value.toInt()
            val turns = match.groups[2]!!.value.split("; ")
                .map { turnString ->
                    val cubes = turnString.split(", ")
                        .associate { cube ->
                            val (countString, color) = cube.split(" ")
                            val count = countString.toInt()

                            color to count
                        }

                    Turn(cubes)
                }

            return Game(id, turns)
        }
    }
}

data class Turn(val cubes: Map<String, Int>) {
    val isValid = cubes.all { (color, count) ->
        count <= BAG_CONTENTS.getOrDefault(color, 0)
    }
}

private val GAME_REGEX = Regex("Game (\\d+): (.+)")

private fun part1(input: String): Int {
    return input.lines()
        .filterNot { it.isBlank() }
        .map { Game.parse(it) }
        .filter { it.isValid }
        .sumOf { it.id }
}

private fun part2(input: String): Int {
    return input.lines()
        .filterNot { it.isBlank() }
        .map { Game.parse(it) }
        .sumOf { it.power }
}

class Day2 : BaseDay() {

    @Test
    fun part1() {
        assertEquals(8, part1(SAMPLE_INPUT))
        assertEquals(2085, part1(INPUT))
    }

    @Test
    fun part2() {
        assertEquals(2286, part2(SAMPLE_INPUT))
        assertEquals(79315, part2(INPUT))
    }
}
