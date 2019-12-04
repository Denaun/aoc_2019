package day4

object Password {
  def isValid(pass: String)(implicit part1: Boolean=false): Boolean = {
    if (pass.length() != 6) {
      return false
    }
    val stream = pass.map(_.asDigit)
    if (part1) {
      if (stream
            .sliding(2)
            .find { d =>
              d(0) == d(1)
            }
            .isEmpty) {
        return false
      }
    } else {
      if (stream
            .prepended(Int.MinValue) // Invalid bound values
            .appended(Int.MinValue)
            .sliding(4)
            .find { d =>
              d(0) != d(1) && d(1) == d(2) && d(2) != d(3)
            }
            .isEmpty) {
        return false
      }
    }
    if (stream
          .sliding(2)
          .find { d =>
            d(0) > d(1)
          }
          .nonEmpty) {
      return false
    }
    return true
  }
}
