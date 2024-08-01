#!/bin/sh

DIR_PATH="./target/img-processing-rust-0.1.0.jar"


echo Jar path: $DIR_PATH

java-pack -r build

mvn deploy:deploy-file -Dfile=$DIR_PATH -DgroupId=image.rs -DartifactId=img-processing-rust -Dversion=0.1.0 -DrepositoryId=local-maven-repo -Durl=file:../local-maven-repo/ -DupdateReleaseInfo=true -Dpackaging=jar
