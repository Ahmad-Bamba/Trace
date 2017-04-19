#pragma once

#include <fstream>
#include <vector>
#include <string>
#include "PlaybackStruct.h"

class PlaybackWriter {
	std::ofstream m_file;

public:
	PlaybackWriter(std::vector<PlaybackStruct> marks, std::string path) {
		m_marks = marks;
		m_file.open(path, m_file.trunc | m_file.out);
	}

	void write() {
		for (auto mark : m_marks)
			m_file << mark.t << "," << mark.vl << "," << mark.vr << std::endl;
		m_file.close();
	}

	std::vector<PlaybackStruct> m_marks;
};
