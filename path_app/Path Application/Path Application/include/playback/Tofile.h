#pragma once

#include <fstream>
#include <vector>
#include <string>
#include <iostream>
#include "PlaybackStruct.h"

class PlaybackWriter {
	std::ofstream m_file;

public:
	PlaybackWriter(std::vector<PlaybackStruct> marks, std::string path) {
		m_marks = marks;
		m_file.open(path, m_file.trunc | m_file.out);
	}

	void force_scale(double scale) {
		// xmax * something = scale
		// something = scale / xmax

		double maxl = 0, maxr = 0, constl = 0, constr = 0;

		for (int_fast16_t i = 1; i < m_marks.size(); i++) {
			if (m_marks[i].vl > m_marks[i - 1].vl)
				maxl = m_marks[i].vl;

			if (m_marks[i].vr > m_marks[i - 1].vr)
				maxr = m_marks[i].vl;
		}

		if (maxl == 0 || maxr == 0)
			std::cout << "Unknown error: refusing to scale" << std::endl;
		else {
			std::cout << maxl << " " << maxr << std::endl;
			constl = scale / maxl;
			constr = scale / maxr;

			for (auto& mark : m_marks) {
				mark.vl *= constl;
				mark.vr *= constr;
			}

			// double math isn't accurate, so trim bad math
			// also 0 out ridiculously small inputs
			for (auto& mark : m_marks) {
				if (mark.vl > scale)
					mark.vl = scale;
				else if (std::abs(mark.vl) < 0.001)
					mark.vl = 0;

				if (mark.vr > scale)
					mark.vr = scale;
				else if (std::abs(mark.vr) < 0.001)
					mark.vr = 0;
			}
		}
	}

	void write() {
		for (auto mark : m_marks)
			m_file << mark.t << "," << mark.vl << "," << mark.vr << std::endl;
		m_file.close();
	}

	std::vector<PlaybackStruct> m_marks;
};
