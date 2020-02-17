document.addEventListener('DOMContentLoaded', function() {
  var player = document.getElementById('player');
  console.log(player);
  player.addEventListener("ended", function(e){
    var data = get_next_song(player);
    if (data == null) {
      set_src("empty", "empty", player);
    } else {
      set_src(data.url, data.name, player);
    }
  }, false);

  setInterval(function(){
    check_if_empty(player); 
  }, 1000);
})

function set_src(url, title, player) {
  console.log('Next song playing is: ' + title);
  document.getElementById('now-playing').innerHTML = ("Now Playing: " + title);
  player.setAttribute('src', url);
  player.load();
  player.play();
}

function check_if_empty(player) {
  console.log(player.currentSrc.replace(location.origin, "") == '/empty');
  if (player.currentSrc.replace(location.origin, "") == '/empty') {
    var data = get_next_song(player);
    if (data == null) {
      set_src("empty", "empty", player);
      return
    } else {
      set_src(data.url, data.name, player);
    }
  } else {
    return
  }
}

function get_next_song(player) {
  var request = new XMLHttpRequest();
  request.open ('GET', location.origin + '/api/get_next_song', false);
  request.send();
  if (request.status === 200) {
    console.log(JSON.parse(request.response));
    return JSON.parse(request.response);
  } else {
    console.log('Response failed');
  }
}
