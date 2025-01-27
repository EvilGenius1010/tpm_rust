#!/bin/bash

# Create TPM state directory if it doesn't exist
# Double check if this is needed.
mkdir -p /var/lib/swtpm

# Startup command
swtpm socket --tpmstate dir=/var/lib/swtpm --ctrl type=tcp,port=2322 --server type=tcp,port=2321 --flags not-need-init --tpm2 & SWTPM_PID=$!


# Wait for sometime for the swtpm socket to initialise
sleep 2s

# Run tpm2_startup command
tpm2_startup --tcti=swtpm:port=2321 -c

# Wait for swtpm to exit
wait $SWTPM_PID
