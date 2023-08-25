#include <iostream>
#include <string>

class Student {
public:
	std::string Name;
	int Age;
	char Gender;
	float Grade;
};

void printStudents(Student* students, int size) {
	for (int i = 0; i < size; i++) {
		std::cout << "Student " << i + 1 << "\t";
		std::cout << students[i].Name << "\t";
		std::cout << students[i].Age << "\t";
		std::cout << students[i].Gender << "\t";
		std::cout << students[i].Grade << "\t";
	}
}

int main() {
	std::cout << "Hello World!\n";

	int size;

	// Prompt the user for the number of students
	std::cout << "Number of students: ";
	std::cin >> size;

	// Input validation
	if (size <= 0) {
		std::cout << "Invalid input. Number of students must be greater than 0.\n";
		return 1; // Exit the program with an error code
	}

	// Allocate memory for an array of Student objects
	Student* students = new Student[size];

	// Populate student information
	for (int i = 0; i < size; i++) {
		std::cout << "Enter information for Student " << i + 1 << ":\n";
		std::cout << "Name: ";
		std::cin >> students[i].Name;
		std::cout << "Age: ";
		std::cin >> students[i].Age;
		std::cout << "Gender (M/F): ";
		std::cin >> students[i].Gender;
		std::cout << "Grade: ";
		std::cin >> students[i].Grade;
	}

	// Process student data or perform other operations as needed
	printStudents(students, size);

	// Deallocate memory when you're done with it
	delete[] students;

	return 0; // Exit the program successfully
}
