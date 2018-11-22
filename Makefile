gogourl = https://raw.githubusercontent.com/gogo/protobuf/master/gogoproto/gogo.proto
gogooutput = protos/github.com/gogo/protobuf/gogoproto/gogo.proto

tmbaseurl = https://raw.githubusercontent.com/tendermint/tendermint/master
tmkv = protos/github.com/tendermint/tendermint/libs/common/common.proto
tmmerkle = protos/github.com/tendermint/tendermint/crypto/merkle/merkle.proto

update-proto:
	curl $(gogourl) > $(gogooutput)
	curl $(tmbaseurl)/abci/types/types.proto > src/types.proto
	curl $(tmbaseurl)/libs/common/types.proto > $(tmkv)
	curl $(tmbaseurl)/crypto/merkle/merkle.proto > $(tmmerkle)
	@echo "                   ~ NOTE ~                    "
	@echo "**** Rename common/types.proto to common/common.proto in src/types.proto!! ****"
