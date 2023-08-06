import init, { World, Direction, GameStatus } from "snake_game";

init().then((wasm) => {
  const CELL_SIZE = 40;
  const WORLD_WIDTH = 14;
  const snakeSpawnIdx = Date.now() % (WORLD_WIDTH * WORLD_WIDTH);
  const world = World.new(WORLD_WIDTH, snakeSpawnIdx);

  const gameControlBtn = document.getElementById("game-control-btn");
  const gameStatus= document.getElementById("game-status");
  const points= document.getElementById("points");
 
  const canvas = <HTMLCanvasElement>document.getElementById("snake-canvas");
  const ctx = canvas.getContext("2d");
  const worldWidth = world.width();

  canvas.height = CELL_SIZE * worldWidth;
  canvas.width = CELL_SIZE * worldWidth;


  function timeToStartTheGame(){
    const status = world.game_status();


    if(status == undefined){
    gameControlBtn.textContent = "Playing...";  
      world.game_start();
      play();
    }else{
      location.reload(); 
    }
  }
  gameControlBtn.addEventListener("click", (e)=>{
    timeToStartTheGame();
  })
  document.addEventListener("keydown", (e) => {
    switch (e.code) {
      case "Enter":{
        timeToStartTheGame();
        return;
      }
    }
  });
  document.addEventListener("keydown", (e) => {
    var commad;
    switch (e.code) {
      case "ArrowUp": {
        commad = Direction.Up;
        break;
      }
      case "ArrowLeft": {
        commad = Direction.Left;
        break;
      }
      case "ArrowRight": {
        commad = Direction.Right;
        break;
      }
      case "ArrowDown": {
        commad = Direction.Down;
        break;
      }
      default: {
        return;
      }
    }

    world.change_snake_dir(commad);
  });

  function drowWorld() {
    ctx.beginPath();

    for (let x = 0; x <= worldWidth; x++) {
      ctx.moveTo(CELL_SIZE * x, 0);
      ctx.lineTo(CELL_SIZE * x, worldWidth * CELL_SIZE);
    }

    for (let y = 0; y <= worldWidth; y++) {
      ctx.moveTo(0, y * CELL_SIZE);
      ctx.lineTo(worldWidth * CELL_SIZE, y * CELL_SIZE);
    }

    ctx.stroke();
  }

  function drawSnakeCell(snakeIdx: number, color: string = "#000000") {
    const col = snakeIdx % worldWidth;
    const row = Math.floor(snakeIdx / worldWidth);

    ctx.beginPath();

    ctx.fillStyle = color;

    ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);

    ctx.stroke();
  }

  function drowSnake() {
    const snakeCellPtr = world.snake_cells();
    const snakeLen = world.snake_len();
    const snakeCells = new Uint32Array(
      wasm.memory.buffer,
      snakeCellPtr,
      snakeLen
    );

    snakeCells
    .filter((cell, idx) => idx === 0 || cell !== snakeCells[0])
    .forEach((cell, i) => {
      drawSnakeCell(cell, i === 0 ? "#7878db" : "#000000");
    });
  }

  function drowReward(){
    const rewardIdx = world.reaward_cell();

    drawSnakeCell(rewardIdx, "#ff0000")
  }

  function drowGameStatus(){
    gameStatus.textContent = world.get_display_status();
    points.textContent = world.points().toString();
  }

  function paint() {
    drowWorld();
    drowSnake();
    drowReward();
    drowGameStatus(); 
  }

  function play() {

    if(world.game_status() == GameStatus.Lost || world.game_status() == GameStatus.Lost){
      gameControlBtn.textContent = "Reset";
      return;
    }

    const fps = 15;
    setTimeout(() => {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      world.step();
      paint();
      // the method takes a callback to invoked before the next repaint
      requestAnimationFrame(play);
    }, 1000 / fps);
  }

  paint();
});
