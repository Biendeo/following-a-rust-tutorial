Action sleepLambda = () => {
	Console.WriteLine("This is a lambda!");
	Thread.Sleep(1000);
	Console.WriteLine("And the lambda is still going!");
};

sleepLambda();

Func<int, int> lambdaSyntax1 = (int x) => { return x + 1; };
Func<int, int> lambdaSyntax2 = (int x) => x + 1;

Console.WriteLine($"Both functions have the same result: {lambdaSyntax1(5)} and {lambdaSyntax2(5)}");

int y = 5;

Action capturingLambda = () => y += 5;

capturingLambda();

Console.WriteLine($"Lambdas can borrow and modify their outer scope: {y}");

Func<List<int>, string> expandList = (List<int> v) => string.Format("[{0}]", string.Join(", ", v));

List<int> list = new() { 1, 2, 3 };
Console.WriteLine($"Before defining lambda: {expandList(list)}");
Action<List<int>> mutablyAddsToList = (List<int> v) => v.Add(7);
Console.WriteLine($"After defining lambda: {expandList(list)}");
mutablyAddsToList(list);
Console.WriteLine($"After calling lambda: {expandList(list)}");

int total = list.Sum();
Console.WriteLine($"The list total is {total}");

List<int> mapping = list.Select(x => x * 50).ToList();
Console.WriteLine($"The mapped list is now {expandList(mapping)}");
IEnumerable<int> mappingInLinq = from x in list select x * 50;
Console.WriteLine($"This syntax also results in the same list: {expandList(mappingInLinq.ToList())}");

List<int> filtering = list.Where(x => x <= 2).ToList();
Console.WriteLine($"The filtered list is now {expandList(filtering)}");
IEnumerable<int> filteringInLinq = from x in list where x <= 2 select x;
Console.WriteLine($"This syntax also results in the same list: {expandList(filteringInLinq.ToList())}");