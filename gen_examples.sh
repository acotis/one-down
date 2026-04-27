
for grid in "tiny" "incomplete" "up" "up_incomplete" "pip_pip_cutie"
do
    ./run.sh examples/${grid}.txt
    mv puzzle.png examples/${grid}_output_puzzle.png
    mv answer_key.png examples/${grid}_output_answer_key.png
done

