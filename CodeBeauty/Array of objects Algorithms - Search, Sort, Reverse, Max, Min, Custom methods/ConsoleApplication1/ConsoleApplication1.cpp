#include <iostream>
#include <string>
#include <algorithm>

class Student {
public:
    std::string Name;
    int Age;
    char Gender;
    float ProgrammingGrade;

    // Constructor with member initialization
    Student(std::string name, int age, char gender, float programmingGrade)
        : Name(name), Age(age), Gender(gender), ProgrammingGrade(programmingGrade) {
    }
};

bool compareByGrade(Student s1, Student s2) {
    return s1.ProgrammingGrade < s2.ProgrammingGrade;
}

bool compareByName(const Student& s1, const Student& s2) {
    return s1.Name < s2.Name;
}

int main() {
    std::cout << "Hello World!\n";

    // Creating an array of Student objects and initializing them
    Student students[5] = {
        {"John", 20, 'm', 9.0f},
        {"Alice", 21, 'f', 8.5f},
        {"Bob", 19, 'm', 7.8f},
        {"Eve", 22, 'f', 9.5f},
        {"Charlie", 20, 'm', 8.0f}
    };

    // Accessing and printing the properties of the students
    for (int i = 0; i < 5; i++) {
        std::cout << "Student " << i + 1 << ":\n";
        std::cout << "Name: " << students[i].Name << "\n";
        std::cout << "Age: " << students[i].Age << "\n";
        std::cout << "Gender: " << students[i].Gender << "\n";
        std::cout << "Programming Grade: " << students[i].ProgrammingGrade << "\n\n";
    }

    std::sort(students, students+5, compareByGrade);

    for (size_t i = 0; i < 4; i++)
    {
        std::cout << students[i].Name << std::endl;
    }

    return 0;
}
