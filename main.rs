let grid = [
  ['A','B','C','E'],
  ['S','F','C','S'],
  ['A','D','E','E']
]

const moves = [-1, 1, -4, 4]

// adjacency list generator function
function adjacencyListFunctionGenerator(validMoves) {
  return function (board) {
    const map = new Map()
    sample = board.reduce((total, curr) => total.concat(curr))
    
    for (let indx = 0; indx < sample.length; indx++) {
      for (let j = 0; j < validMoves.length; j++) {
        if (sample[indx+validMoves[j]]) {
          let arr = map[indx] || []
          arr.push(sample[indx + validMoves[j]])
          map[indx] = arr
        }
      }
    }
    return map
  }
}
const createAdjacency = adjacencyListFunctionGenerator(moves)

// driver code
console.log(createAdjacency(grid))
