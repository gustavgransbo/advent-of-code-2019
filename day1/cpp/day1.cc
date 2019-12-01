#include <iostream>
#include <iterator>
#include <vector>
#include <algorithm>

using namespace std;

int fuel_requirement_of_fuel(int const &fuel)
{
    int requirement = (fuel / 3) - 2;
    if(requirement <= 0) return 0;
    return requirement += fuel_requirement_of_fuel(requirement);
}

int main()
{
	int total_fuel_needed {};
    int total_fuel_needed_for_fuel {};
	for_each(istream_iterator<int>(cin), istream_iterator<int>(),
		[&total_fuel_needed, &total_fuel_needed_for_fuel](int const &weight)
		{
            int fuel_needed = (weight / 3) - 2;
			total_fuel_needed += fuel_needed;
            total_fuel_needed_for_fuel += fuel_requirement_of_fuel(fuel_needed);
		});

    cout << "Answer part 1: " << total_fuel_needed << endl;
    cout << "Answer part 2: " << total_fuel_needed + total_fuel_needed_for_fuel;
}
