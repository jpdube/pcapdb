echo "Clean DB and pcap files"
rm -f ./db/index.db
rm -f ./db/index.db-journal
rm -f ./db/pcap/*.pcap
echo "filename_seq: 0" > ./db/db_config.yaml
echo "Clean-up terminated"
