set -xe

cargo build --bin proof 

PROCS=()
./target/debug/proof -p groth16 -c squaring --computation-size 3 mpc --hosts ./data/2 --alg spdz --party 0 &
pid=$!
PROCS+=($pid)

./target/debug/proof -p groth16 -c squaring --computation-size 3 mpc --hosts ./data/2 --alg spdz --party 1 > /dev/null 2>&1 &
pid=$
PROCS+=($pid)

for pid in ${PROCS}
do
  wait $pid
done
