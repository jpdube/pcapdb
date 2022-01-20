echo "Clean DB and pcap files"
rm -f ./db/packets.db
rm -f ./db/packets.db-journal
rm -f ./db/pcap/*.pcap
echo "filename_seq: 0" > ./db/db_config.yaml
echo "Clean-up terminated"
