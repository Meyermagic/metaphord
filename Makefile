

.PHONY : all
all: metaphord

.PHONY : metaphord
metaphord:
	rustc -g -L deps -o metaphord src/main.rs

.PHONY : clean
clean:
	rm metaphord
