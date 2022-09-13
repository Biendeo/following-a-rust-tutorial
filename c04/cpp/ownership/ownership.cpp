#include <iostream>
#include <string>

void steal_the_string(std::string s);
void borrow_the_string(std::string& s);

class ownership_example {
	public:
	ownership_example(const std::string& s) {
		std::cout << "This class with \"" << s << "\" is constructing..." << std::endl;
		this->s = s;
	}

	~ownership_example() {
		std::cout << "This class with \"" << s << "\" is deconstructing..." << std::endl;
	}

	std::string s;
};

int main(int argc, char* argv[]) {
	{
		char* s = "hello";
	}
	// s goes out of scope.

	std::string s1 = std::string("hello"); // Technically the same ends as saying std::string s1 = "hello", C++ implicitly would construct.
	char* s2 = "hello";
	char s3[16] = "hello"; // This is kinda spooky because this is writable but has different properties to the above one.

	std::cout << "The original values are..." << std::endl;
	std::cout << s1 << std::endl;
	std::cout << s2 << std::endl;
	std::cout << s3 << std::endl;

	std::cout << "Now to do some modifying..." << std::endl;

	s1[3] = '@'; // This is fine.
	std::cout << s1 << std::endl;
	// s2[3] = '@'; // This segfaults because it's not modifiable.
	std::cout << s2 << std::endl;
	s3[3] = '@'; // This is fine.
	std::cout << s3 << std::endl;

	steal_the_string(s1);
	std::cout << "Nope, I can print it still: " << s1 << std::endl;
	std::cout << "And the address of s1 is: " << &s1 << std::endl;

	borrow_the_string(s1);
	std::cout << "This was a reference and thus modified it: " << s1 << std::endl;

	{
		// Note how the constructor gets called.
		ownership_example x{"Example 1"};
		// And the deconstructor now gets called.
	}

	{
		ownership_example x{"Example 2"};
		ownership_example& r = x;
		{
			ownership_example x2{"Example 2 in the scope"};
			r = x2; // Note this is doing a copy assignment and not a reference assignment!
		}
		std::cout << "Still have a reference to r: " << r.s << std::endl;
		std::cout << "But it actually modified x: " << x.s << std::endl;
	}

	{
		ownership_example* p;
		{
			ownership_example x{"Example 3"};
			p = &x;
		}
		std::cout << "Is p valid here: " << p->s << std::endl; // This...might be compiler-specific.

		p = new ownership_example{"Exmaple 4"};
		
		std::cout << "p is definitely valid: " << p->s << std::endl;

		// Make sure you free anything manually allocated.
		delete p;
	}

	return 0;
}

void steal_the_string(std::string s) {
	std::cout << "Did this function steal the std::string: " << s << std::endl;
	std::cout << "But the address of s is: " << &s << std::endl;
	s[0] = 'H';
}

void borrow_the_string(std::string& s) {
	std::cout << "Did this function borrow the std::string: " << s << std::endl;
	std::cout << "But the address of s is: " << &s << std::endl;
	s[0] = 'H';
}