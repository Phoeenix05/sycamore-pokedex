#!/usr/bin/env fish

set PROJECT_NAME sycamore-pokedex
set BUILD_DIR ./dist

trunk build --release

vercel link --project $PROJECT_NAME -y --cwd $BUILD_DIR
cd $BUILD_DIR
vercel deploy
