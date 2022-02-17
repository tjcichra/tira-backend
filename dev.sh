#!/bin/bash
sleep 3 && diesel setup && diesel migration run && cargo run