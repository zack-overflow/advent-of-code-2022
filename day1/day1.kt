// import io.File;

fun main() {
         parseInputFile("input1.txt");
}

fun parseInputFile(fileName: String) = File(fileName).forEachLine { println(it) }
