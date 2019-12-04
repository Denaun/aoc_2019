package day4

object Password {
  def isValid(pass: String): Boolean = {
    if (pass.length() != 6) {
      return false
    }
    val stream = pass.map { c =>
      c.asDigit
    }
    val adjacent = stream.zip(stream.tail)
    if (adjacent.find { d =>
          d._1 == d._2
        }.isEmpty) {
      return false
    }
    if (adjacent.find { d =>
          d._1 > d._2
        }.nonEmpty) {
      return false
    }
    return true
  }
}
