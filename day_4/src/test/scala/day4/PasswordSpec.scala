package day4

import org.scalatest._

class PasswordSpec extends FlatSpec with Matchers {
  "The Password object" should "succeed with valid passwords" in {
    Password.isValid("111111") shouldBe true
    Password.isValid("111123") shouldBe true
    Password.isValid("122345") shouldBe true
  }

  it should "fail with invalid passwords" in {
    Password.isValid("135679") shouldBe false
    Password.isValid("223450") shouldBe false
    Password.isValid("123789") shouldBe false
    Password.isValid("11111") shouldBe false
    Password.isValid("1111111") shouldBe false
  }

  it should "solve part 1" in {
    val start = 147981
    val end = 691423
    Stream
      .from(start)
      .take(end - start)
      .map { n =>
        n.toString()
      }
      .count { p =>
        Password.isValid(p)
      } shouldBe 1790
  }
}
