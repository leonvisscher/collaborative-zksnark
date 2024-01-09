set -xe

cargo build 

PROCS=()
./target/debug/cp --party 0 1 2 6 &
pid=$!
PROCS+=($pid)

./target/debug/cp --party 1 3 7 30 &
pid=$
PROCS+=($pid)

for pid in ${PROCS}
do
  wait $pid
done
