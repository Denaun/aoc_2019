package day4

import org.scalatest._
import org.scalatest.prop.TableDrivenPropertyChecks

class PasswordSpec
    extends FlatSpec
    with Matchers
    with TableDrivenPropertyChecks {
  val booleans = Table("v", true, false)

  "The Password object" should "succeed with valid passwords" in {
    forAll(booleans) { implicit part1 =>
      Password.isValid("122345") shouldBe true
      Password.isValid("112233") shouldBe true
      Password.isValid("111122") shouldBe true
    }

    implicit val part1 = true
    Password.isValid("111111") shouldBe true
    Password.isValid("111123") shouldBe true
    Password.isValid("123444") shouldBe true
  }

  it should "fail with invalid passwords" in {
    Password.isValid("111111") shouldBe false
    Password.isValid("111123") shouldBe false
    Password.isValid("123444") shouldBe false

    forAll(booleans) { implicit part1 =>
      Password.isValid("135679") shouldBe false
      Password.isValid("223450") shouldBe false
      Password.isValid("123789") shouldBe false
      Password.isValid("11111") shouldBe false
      Password.isValid("1111111") shouldBe false
    }
  }

  it should "solve part 1" in {
    implicit val part1 = true
    val start = 147981
    val end = 691423
    Stream
      .from(start)
      .take(end - start)
      .map(_.toString)
      .count(Password.isValid _) shouldBe 1790
  }

  it should "solve part 2" in {
    val start = 147981
    val end = 691423
    Stream
      .from(start)
      .take(end - start)
      .map(_.toString)
      .count(Password.isValid _) shouldBe 1206
  }
}
