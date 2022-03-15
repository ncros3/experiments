window.onload = function () {
    var canvas = document.getElementById("target");
    var ctx = canvas.getContext("2d");
    var img = document.getElementById("front");
    ctx.drawImage(img, 0, 0);
};
{
    var rayon_1 = 100;
    var circle_evolve_1 = true;
    function drawCircle() {
        var canvas = document.getElementById("target");
        var ctx = canvas.getContext("2d");
        ctx.clearRect(0, 0, canvas.clientWidth, canvas.clientHeight);
        if (circle_evolve_1 === true) {
            if (rayon_1 >= 300) {
                rayon_1 = 300;
                circle_evolve_1 = false;
            }
            else {
                rayon_1 += 2;
            }
        }
        else {
            if (rayon_1 <= 100) {
                rayon_1 = 100;
                circle_evolve_1 = true;
            }
            else {
                rayon_1 -= 2;
            }
        }
        ctx.beginPath();
        ctx.arc(1000, 600, rayon_1, 0, 2 * Math.PI);
        ctx.stroke();
    }
    setInterval(drawCircle, 30);
}
