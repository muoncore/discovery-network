
all: package

lib:
	mkdir -p discovery-network/src/main/resources/linux-x86-64/
	cp ../target/release/libmuon_discovery_net.so discovery-network/src/main/resources/linux-x86-64/

package: clean lib
	pwd
	./gradlew artifactoryPublish

clean:
	./gradlew clean

test:
	./gradlew test

version:
	echo version=$(VERSION)>gradle.properties
	echo group=io.muoncore>>gradle.properties
	echo rootOnly=true>>gradle.properties
	echo exclude=>>gradle.properties
	git add gradle.properties
