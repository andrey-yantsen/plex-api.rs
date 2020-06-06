use crate::LibraryMediaContainer;
use serde_json::from_str;

#[test]
fn decode_library_sections() {
    let s = r##"
{"size":3,"allowSync":false,"identifier":"com.plexapp.plugins.library","mediaTagPrefix":"/system/bundle/media/flags/","mediaTagVersion":1578431856,"title1":"Plex Library","Directory":[{"allowSync":true,"art":"/:/resources/movie-fanart.jpg","composite":"/library/sections/1/composite/1579536593","filters":true,"refreshing":false,"thumb":"/:/resources/movie.png","key":"1","type":"movie","title":"Movies","agent":"com.plexapp.agents.imdb","scanner":"Plex Movie Scanner","language":"en","uuid":"5ac3b52d-e852-488e-9f9c-ca6988daa269","updatedAt":1579536650,"createdAt":1506579881,"scannedAt":1579536593,"content":true,"directory":true,"contentChangedAt":1690867,"Location":[{"id":1,"path":"/Volumes/External/Movies"}]},{"allowSync":true,"art":"/:/resources/movie-fanart.jpg","composite":"/library/sections/14/composite/1579536595","filters":true,"refreshing":false,"thumb":"/:/resources/movie.png","key":"14","type":"movie","title":"Stand-ups","agent":"com.plexapp.agents.imdb","scanner":"Plex Movie Scanner","language":"en","uuid":"4554e3e4-a8f0-4b2c-9406-0144e22b7e64","updatedAt":1579536650,"createdAt":1548180402,"scannedAt":1579536595,"content":true,"directory":true,"contentChangedAt":1688871,"Location":[{"id":15,"path":"/Volumes/External/Stand-ups"}]},{"allowSync":true,"art":"/:/resources/show-fanart.jpg","composite":"/library/sections/2/composite/1579536598","filters":true,"refreshing":false,"thumb":"/:/resources/show.png","key":"2","type":"show","title":"TV Shows","agent":"com.plexapp.agents.thetvdb","scanner":"Plex Series Scanner","language":"en","uuid":"cbf5ffe4-8b58-4019-81b8-60aaeceb0f78","updatedAt":1579536650,"createdAt":1506579911,"scannedAt":1579536598,"content":true,"directory":true,"contentChangedAt":1692548,"Location":[{"id":2,"path":"/Volumes/External/TV Shows"}]}]}
    "##;

    let mc = from_str::<LibraryMediaContainer>(s);
    assert!(
        dbg!(&mc).is_ok(),
        "Unable to deserialize library sections: {:?}",
        mc.err()
    );
}

#[test]
fn decode_library_on_deck() {
    let s = r##"
{"size":1,"allowSync":false,"identifier":"com.plexapp.plugins.library","mediaTagPrefix":"/system/bundle/media/flags/","mediaTagVersion":1578431856,"mixedParents":true,"Metadata":[{"allowSync":true,"librarySectionID":2,"librarySectionTitle":"TV Shows","librarySectionUUID":"cbf5ffe4-8b58-4019-81b8-60aaeceb0f78","ratingKey":"12285","key":"/library/metadata/12285","skipParent":true,"parentRatingKey":"10092","grandparentRatingKey":"3884","guid":"com.plexapp.agents.thetvdb://71663/31/11?lang=en","parentGuid":"com.plexapp.agents.thetvdb://71663/31?lang=en","grandparentGuid":"com.plexapp.agents.thetvdb://71663?lang=en","type":"episode","title":"Hail to the Teeth","grandparentKey":"/library/metadata/3884","parentKey":"/library/metadata/10092","librarySectionKey":"/library/sections/2","grandparentTitle":"The Simpsons","parentTitle":"Season 31","contentRating":"TV-PG","summary":"Homer and Marge attend Artie Ziff’s wedding and become quite uncomfortable when they realize that his bride-to-be is a clone of Marge. Meanwhile, Lisa grapples with the misogynistic implications of the immediate popularity she receives after getting her new Invisalign braces.","index":11,"parentIndex":31,"year":2020,"thumb":"/library/metadata/12285/thumb/1578283608","art":"/library/metadata/3884/art/1578283608","parentThumb":"/library/metadata/10092/thumb/1578283608","grandparentThumb":"/library/metadata/3884/thumb/1578283608","grandparentArt":"/library/metadata/3884/art/1578283608","grandparentTheme":"/library/metadata/3884/theme/1578283608","duration":1293336,"originallyAvailableAt":"2020-01-05","addedAt":1578283572,"updatedAt":1578283608,"Media":[{"id":29879,"duration":1293336,"bitrate":8903,"width":1920,"height":1080,"aspectRatio":1.78,"audioChannels":2,"audioCodec":"aac","videoCodec":"h264","videoResolution":"1080","container":"mkv","videoFrameRate":"24p","audioProfile":"he-aac","videoProfile":"high","Part":[{"id":29892,"key":"/library/parts/29892/1578277841/file.mkv","duration":1293336,"file":"/Volumes/External/TV Shows/The Simpsons (1989)/the.simpsons.s31e11.1080p.web.mkv","size":730248189,"audioProfile":"he-aac","container":"mkv","indexes":"sd","videoProfile":"high","Stream":[{"id":109823,"streamType":1,"default":true,"codec":"h264","index":0,"bitrate":4388,"bitDepth":8,"chromaLocation":"center","chromaSubsampling":"4:2:0","closedCaptions":"1","codedHeight":"1088","codedWidth":"1920","colorPrimaries":"bt709","colorRange":"tv","colorSpace":"bt709","colorTrc":"bt709","frameRate":23.976,"hasScalingMatrix":false,"height":1080,"level":40,"profile":"high","refFrames":2,"scanType":"progressive","width":1920,"displayTitle":"1080p (H.264)"},{"id":109825,"streamType":2,"selected":true,"default":true,"codec":"aac","index":1,"channels":2,"bitrate":127,"audioChannelLayout":"stereo","profile":"he-aac","samplingRate":48000,"displayTitle":"Unknown (HE-AAC Stereo)"},{"id":109824,"streamType":3,"selected":true,"codec":"eia_608","index":0,"bitrate":4388,"embeddedInVideo":"1","displayTitle":"Unknown (EIA_608)"}]}]}],"Director":[{"id":62586,"filter":"director=62586","tag":"Mark Kirkland"}],"Writer":[{"id":62585,"filter":"writer=62585","tag":"Elisabeth Kiernan Averick"}]}]}
    "##;

    let mc = from_str::<LibraryMediaContainer>(s);
    assert!(
        dbg!(&mc).is_ok(),
        "Unable to deserialize library on deck: {:?}",
        mc.err()
    );
}
