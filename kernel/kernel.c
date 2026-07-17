

extern void _start(int* fb, unsigned long w) {

	for (int i = 0; i < 100; i++) {
		*(fb + i * w + i) = 0xffffff;
	}

	for (;;) {}

}
