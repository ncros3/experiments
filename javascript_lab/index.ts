window.onload = function () {
    let canvas = <HTMLCanvasElement> document.getElementById("target");
    let ctx = canvas.getContext("2d");
    let img = <HTMLImageElement> document.getElementById("front");
    ctx.drawImage(img, 0, 0);
}

{
    let rayon = 100;
    let circle_evolve = true;
    
    function drawCircle() {
        let canvas = <HTMLCanvasElement> document.getElementById("target");
        let ctx = canvas.getContext("2d");
    
        ctx.clearRect(0,0,canvas.clientWidth, canvas.clientHeight);
    
        if (circle_evolve === true) {
    
            if (rayon >= 300) {
                rayon = 300;
                circle_evolve = false;
            } else {
                rayon += 2;
            }
    
        } else {
    
            if (rayon <= 100) {
                rayon = 100;
                circle_evolve = true;
            } else {
                rayon -= 2;
            }
    
        }
    
        ctx.beginPath();
        ctx.arc(1000, 600, rayon, 0, 2 * Math.PI);
        ctx.stroke();
    }

    setInterval(drawCircle, 30);
}