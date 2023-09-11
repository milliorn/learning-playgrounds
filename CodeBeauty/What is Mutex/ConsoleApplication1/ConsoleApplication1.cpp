#include <iostream>
#include <thread>
#include <mutex>

using namespace std;

mutex carMutex;

void driveCar(std::string driverName) {
  lock_guard<mutex> carlock(carMutex);
  std::cout << driverName << " is driving\n" << std::endl;
  this_thread::sleep_for(chrono::seconds(2));
  std::cout << driverName << " is done driving\n" << endl;
}
int main()
{
  thread t1(driveCar, "Scott");
  thread t2(driveCar, "Saldina");

  t1.join();
  t2.join();

  std::cin.get();
}
