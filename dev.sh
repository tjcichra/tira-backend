#!/bin/bash
sleep 3 && sqlx create database && sqlx migrate run && cargo run