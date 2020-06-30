# Origin
version_branch = v0.33.5
tendermint = https://raw.githubusercontent.com/tendermint/tendermint/$(version_branch)

# Outputs
tmkv = protobuf/libs/kv/types.proto
tmmerkle = protobuf/crypto/merkle/merkle.proto
tmabci = protobuf/abci/types/types.proto
third_party = third_party/proto/gogoproto/gogo.proto

# You *only* need to run this to rebuild protobufs from the tendermint source
update-proto:
	curl $(tendermint)/abci/types/types.proto > $(tmabci)
	curl $(tendermint)/libs/kv/types.proto > $(tmkv)
	curl $(tendermint)/crypto/merkle/merkle.proto > $(tmmerkle)
	sed 's@package types;@package abci;@' $(tmabci) > protobuf/abci.proto
	curl $(tendermint)/version/version.go | grep -F -eTMCoreSem -eABCISemVer > version.txt
	curl $(tendermint)/$(third_party) > protobuf/$(third_party)
