#include <pathfinder.h>
#include <iostream>
#include <iterator>
#include <string>
#include <array>

#include <playback/PlaybackStruct.h>
#include <playback/Tofile.h>

#define DT 0.1 // s
#define MAX_SPEED 0.2 // m/s
#define MAX_ACCELERATION 0.5 // m/s^2
#define MAX_JERK 1.0 // m/s^3
#define WHEELBASE_WIDTH 0.09 // m

using fint = int_fast16_t;

inline unsigned get_char_freq(const char* str, size_t size, char ch) {
	unsigned x = 0;
	for(fint i = 0; i < size; i++)
		if(str[i] == ch) x++;
	return x;
}

// goodbye C crap hello C++
std::array<std::string, 3> split_str(const char* str, size_t size) {
	std::array<std::string, 3> split { "",  "", "" };
	size_t on_index = 0;
	for(fint i = 0; i < size; i++) {
		if(str[i] == ',')
			on_index++;
		else
			split[on_index] += str[i];
	}
	return split;
}

// takes waypoints as commandline arguments:
// x, y, and angle in degrees
int main(int argc, const char** argv) {
	if(!(argc > 2)) {
		std::cout << "Format: path_app x,y,angle x,y,angle x,y,angle..." << std::endl;
		std::cout << "Press ENTER to continue" << std::endl;
		std::cin.get();
		return 1;
	}

	Waypoint* points = new Waypoint[argc - 1];

	for(fint i = 1; i < argc; i++) {
		//std::cout << "Debug: formatting argument: " << argv[i] << std::endl;
		if(get_char_freq(argv[i], std::string(argv[i]).size(), ',') != 2) {
			std::cout << "Format error on point input!" << std::endl << "Format: path_app x,y,angle x,y,angle x,y,angle..." << std::endl;
			return 1;
		}

		auto values = split_str(argv[i], std::string(argv[i]).size());
		std::cout << values[0] << "," << values[1] << "," << values[2] << std::endl;

		try {
			points[i - 1] = Waypoint{ std::stod(values[0]), std::stod(values[1]), d2r(std::stod(values[2])) };
		} catch(std::exception& e) {
			std::cout << e.what() << std::endl;
			std::cout << "Invalid token! " << std::endl << "Format: path_app x,y,angle x,y,angle x,y,angle..." << std::endl;
		}
		
	}

	Segment* trajectory;
	size_t trajectory_l;

	std::cout << "Generating path..." << std::endl;

	{
		TrajectoryCandidate candidate;
		pathfinder_prepare(points, argc - 1, FIT_HERMITE_CUBIC, PATHFINDER_SAMPLES_FAST, DT, MAX_SPEED, MAX_ACCELERATION, MAX_JERK, &candidate);
		trajectory_l = candidate.length;

		try {
			std::cout << "Attempting to create trajectory of size: " << trajectory_l << "..." << std::endl;
			trajectory = new Segment[trajectory_l];
		} catch (std::exception& e) {
			std::cout << e.what() << std::endl;
			std::cout << "Fatal error: divergent path!" << std::endl;
			std::cout << "Tip: pick way points that don't change direction so quickly" << std::endl;
			return 1;
		}

		if(pathfinder_generate(&candidate, trajectory) < 0) {
			std::cout << "Fatal error: path could not be generated!" << std::endl;
			return 2;
		}
	}

	std::cout << "Displaying raw data..." << std::endl;
	for(fint i = 0; i < trajectory_l; i++) {
		std::cout << "Coords: (" << trajectory[i].x << "," << trajectory[i].y << ")" << "\t" << "Pos: " << trajectory[i].position << 
			"\t" << "Velocity: " << trajectory[i].velocity << "\t" << "Acceleration: " << trajectory[i].acceleration <<
			"\t" << "Jerk: " << trajectory[i].jerk << "\t" << "Heading (angle): " << r2d(trajectory[i].heading) << std::endl;
	}

	std::cout << "Generating left and right inputs..." << std::endl;

	Segment* left_trajectory = new Segment[trajectory_l];
	Segment* right_trajectory = new Segment[trajectory_l];
	
	pathfinder_modify_tank(trajectory, trajectory_l, left_trajectory, right_trajectory, WHEELBASE_WIDTH);

	std::cout << "Writing to output file..." << std::endl;

	double t = 0;
	std::vector<PlaybackStruct> current_marks;
	for (fint i = 0; i < trajectory_l; i++) {
		PlaybackStruct p;
		p.t = t;
		p.vr = right_trajectory[i].velocity / MAX_SPEED;
		p.vl = left_trajectory[i].velocity / MAX_SPEED;
		current_marks.push_back(p);
		t += left_trajectory[i].dt;
	}

	PlaybackWriter writer(current_marks, "output.txt");
	writer.write();

	std::cout << "Cleaning assets..." << std::endl;

	// cleanup
	delete points;
	delete trajectory;
	delete left_trajectory;
	delete right_trajectory;

	std::cout << "Done!" << std::endl;

	std::cout << "Press ENTER to continue..." << std::endl;
	std::cin.get();

	return 0;
}
