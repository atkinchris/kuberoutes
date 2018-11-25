build:
	sh ./release.sh

run-labels:
	kubectl run --rm -i -t --image=alpine test --labels role=tester -- wget -qO- --timeout=2 http://web

run-nolabels:
	kubectl run --rm -i -t --image=alpine test -- wget -qO- --timeout=2 http://web
