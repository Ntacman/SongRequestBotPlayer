document.addEventListener('DOMContentLoaded', function() {
  var player = document.getElementById('player');
  console.log(player);
  player.addEventListener("ended", function(e){
    var request = new XMLHttpRequest()
    var base = location.origin;
    
    request.open('GET', base + '/api/get_next_song', true)
    
    request.onload = function() {
      var data = JSON.parse(this.response)
      if (request.status >= 200 && request.status < 400) {
        document.getElementById('now-playing').innerHTML = ("Now Playing: " + data.name);
        set_src(data.url, data.name, player);
      } else {
        console.log('error');
      }
    }
  
    request.send();
  }, false);
})

function set_src(url, title, player) {
  console.log('Next song playing is: ' + title);
  player.setAttribute('src', url);
  player.load();
  player.play();
}
