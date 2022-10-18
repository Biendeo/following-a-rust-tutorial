namespace Tests;

using System.Collections;
using Adder;

public class AdderTests {
	[Fact]
	public void AssertTrue() {
		int result = Adder.Add(2, 2);
		Assert.True(result == 4);
	}

	[Fact]
	public void AssertEqual() {
		int result = Adder.Add(2, 2);
		Assert.Equal(4, result);
	}

	[Fact]
	public void AssertNotEqual() {
		int result = Adder.Add(2, 2);
		Assert.NotEqual(5, result);
	}

	[Fact]
	public void AssertThrows() {
		Exception e = Assert.ThrowsAny<Exception>(() => throw new Exception("Hi!"));
		Assert.Equal("Hi!", e.Message);
	}

	[Theory]
	[InlineData(2, 2, 4)]
	[InlineData(5, 10, 15)]
	[InlineData(125012, 5347, 130359)]
	[ClassData(typeof(TheoryData))]
	public void AssertTheory(int left, int right, int expected) {
		int result = Adder.Add(left, right);
		Assert.Equal(expected, result);
	}

	public class TheoryData : IEnumerable<object[]>, IEnumerable {
		IEnumerator<object[]> GetEnumerator() {
			foreach (int x in Enumerable.Range(0, 10000)) {
				yield return new object[] { x, x + 1, 2 * x + 1 };
			}
		}
		
		IEnumerator IEnumerable.GetEnumerator() => this.GetEnumerator();

		IEnumerator<object[]> IEnumerable<object[]>.GetEnumerator() => this.GetEnumerator();
	}
}
