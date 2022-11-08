#include <iostream>
#include <memory>
#include <string>

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
		std::unique_ptr<ownership_example> p = std::make_unique<ownership_example>("This is a unique pointer!");
		// p.get() gets the raw pointer, `->` dereferences and accesses as if it were a typical pointer.
		std::cout << "Doing some things with my pointer at " << &p << std::endl;
	}

	{
		std::shared_ptr<ownership_example> p = std::make_shared<ownership_example>("This is a shared pointer!");
		// Same syntax as unique pointer but it only deconstructs when it has to.
		std::cout << "Doing some things with my pointer at " << &p << std::endl;
		{
			std::weak_ptr<ownership_example> p2 = std::weak_ptr<ownership_example>(p);
			std::cout << "p still exists pointing to \"" << p->s << "\"" << std::endl;
			// Weak pointers have a reference but they don't count. They must be promoted to shared pointers first.
			{
				std::shared_ptr<ownership_example> p3 = p2.lock();
				std::cout << "Now p3 can reference \"" << p3->s << "\"" << std::endl;
			}
			// p2 can tell if an object is safe to derefence with .expired().
		}
	}

	return 0;
}