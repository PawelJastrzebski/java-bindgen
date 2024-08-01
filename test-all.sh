#!/bin/bash
set -e

echo ""
echo "------------------------------java-pack------------------------------"
echo ""
cd ./bin/java-pack && cargo test && cd ../..

echo ""
echo "--------------------------java-bindgen-core---------------------------"
echo ""
cd ./bin/java-bindgen-core && cargo test && cd ../..

echo ""
echo "-------------------------java-bindgen-macro---------------------------"
echo ""
cd ./bin/java-bindgen-macro && cargo test && cd ../..

echo ""
echo "-------------------------examples/test-macro--------------------------"
echo ""
cd ./examples/test-macro && cargo test && java-pack clean && java-pack test && cd ../..

echo ""
echo "---------------------------java-bindgen-------------------------------"
echo ""
cargo test

