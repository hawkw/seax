#!/bin/bash

if [ "$TRAVIS_REPO_SLUG" == "hawkw/seax" ] &&
   [ "$TRAVIS_PULL_REQUEST" == "false" ] &&
   [ ! -z "$TRAVIS_TAG" ]; then

    echo -e "Starting release...\n"

    cargo doc
    .travis/publish.sh
    cargo login $CRATES_TOKEN

    if [[ "$TRAVIS_TAG" =~ "svm-*" ]]; then

        echo -e "Releasing SVM version $TRAVIS_TAG...\n"

        cd seax_svm
        cargo package
        cargo publish
        cd ..

    elif [[ "$TRAVIS_TAG" =~ "scheme-*" ]]; then

        echo -e "Releasing Scheme version $TRAVIS_TAG...\n"

        cd seax_scheme
        cargo package
        cargo publish
        cd ..

    else

        echo -e "Releasing Seax version $TRAVIS_TAG...\n"

        cargo package
        cargo publish

    fi

    echo -e "$TRAVIS_TAG released "

else

    echo -e "Not on a new tag, skipping release...\n"

fi