$(document).ready(function() {
  var origin = location.origin;
  var playlist_url = origin + "/api/playlist"
  $.ajax({url: playlist_url, success: function(result){
    console.log(result);
    $.each(result.items, function(index, item) {
      var element = "<td>" + item.name + "</td>"
      $('#queue tbody').append("<tr>" + element + "</tr>");
    });
  }});

  setInterval(function() {
    location.reload(); 
  }, 5000);
});
