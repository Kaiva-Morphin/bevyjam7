<?xml version="1.0" encoding="UTF-8"?>
<tileset version="1.10" tiledversion="1.11.2" name="miami" tilewidth="18" tileheight="18" tilecount="1024" columns="32">
 <image source="tileset.png" width="576" height="576"/>
 <tile id="0">
  <objectgroup draworder="index" id="2">
   <object id="3" x="13.5" y="18">
    <polygon points="0,0 4.5,-4.5 4.5,0"/>
   </object>
  </objectgroup>
 </tile>
 <tile id="1">
  <objectgroup draworder="index" id="2">
   <object id="2" x="0" y="18">
    <polygon points="0,0 4.5,0 0,-4.5"/>
   </object>
  </objectgroup>
 </tile>
 <tile id="2">
  <objectgroup draworder="index" id="3">
   <object id="2" x="14" y="0" width="4" height="18"/>
   <object id="3" x="0" y="14" width="14" height="4"/>
  </objectgroup>
 </tile>
 <tile id="3">
  <objectgroup draworder="index" id="2">
   <object id="1" x="0" y="0" width="4" height="18"/>
   <object id="2" x="4" y="14" width="14" height="4"/>
  </objectgroup>
 </tile>
 <tile id="32">
  <objectgroup draworder="index" id="2">
   <object id="2" x="13.5" y="0">
    <polygon points="0,0 4.5,4.5 4.5,0"/>
   </object>
  </objectgroup>
 </tile>
 <tile id="33">
  <objectgroup draworder="index" id="2">
   <object id="2" x="-4.5" y="0">
    <polygon points="9,0 4.5,4.5 4.5,0"/>
   </object>
  </objectgroup>
 </tile>
 <tile id="34">
  <objectgroup draworder="index" id="2">
   <object id="1" x="0" y="0" width="18" height="4"/>
   <object id="2" x="14" y="4" width="4" height="14"/>
  </objectgroup>
 </tile>
 <tile id="35">
  <objectgroup draworder="index" id="3">
   <object id="3" x="0" y="0" width="18" height="4"/>
   <object id="5" x="0" y="4" width="4" height="14"/>
  </objectgroup>
 </tile>
 <tile id="64">
  <objectgroup draworder="index" id="2">
   <object id="1" x="0" y="14" width="18" height="4"/>
  </objectgroup>
 </tile>
 <tile id="65">
  <objectgroup draworder="index" id="2">
   <object id="1" x="0" y="14" width="18" height="4"/>
  </objectgroup>
 </tile>
 <tile id="66">
  <objectgroup draworder="index" id="2">
   <object id="1" x="14" y="0" width="4" height="18"/>
  </objectgroup>
 </tile>
 <tile id="67">
  <objectgroup draworder="index" id="2">
   <object id="1" x="0" y="0" width="4" height="18"/>
  </objectgroup>
 </tile>
 <tile id="96">
  <objectgroup draworder="index" id="2">
   <object id="1" x="0" y="0" width="18" height="4"/>
  </objectgroup>
 </tile>
 <tile id="97">
  <objectgroup draworder="index" id="2">
   <object id="1" x="0" y="0" width="18" height="4"/>
  </objectgroup>
 </tile>
 <tile id="98">
  <objectgroup draworder="index" id="2">
   <object id="1" x="14" y="0" width="4" height="18"/>
  </objectgroup>
 </tile>
 <tile id="99">
  <objectgroup draworder="index" id="2">
   <object id="1" x="0" y="0" width="4" height="18"/>
  </objectgroup>
 </tile>
 <tile id="128">
  <objectgroup draworder="index" id="2">
   <object id="1" x="0" y="0" width="18" height="18"/>
  </objectgroup>
 </tile>
 <wangsets>
  <wangset name="walls" type="corner" tile="-1">
   <wangcolor name="" color="#ff0000" tile="-1" probability="1"/>
   <wangtile tileid="0" wangid="0,0,0,1,0,0,0,0"/>
   <wangtile tileid="1" wangid="0,0,0,0,0,1,0,0"/>
   <wangtile tileid="2" wangid="0,1,0,1,0,1,0,0"/>
   <wangtile tileid="3" wangid="0,0,0,1,0,1,0,1"/>
   <wangtile tileid="32" wangid="0,1,0,0,0,0,0,0"/>
   <wangtile tileid="33" wangid="0,0,0,0,0,0,0,1"/>
   <wangtile tileid="34" wangid="0,1,0,1,0,0,0,1"/>
   <wangtile tileid="35" wangid="0,1,0,0,0,1,0,1"/>
   <wangtile tileid="64" wangid="0,0,0,1,0,1,0,0"/>
   <wangtile tileid="65" wangid="0,0,0,1,0,1,0,0"/>
   <wangtile tileid="66" wangid="0,1,0,1,0,0,0,0"/>
   <wangtile tileid="67" wangid="0,0,0,0,0,1,0,1"/>
   <wangtile tileid="96" wangid="0,1,0,0,0,0,0,1"/>
   <wangtile tileid="97" wangid="0,1,0,0,0,0,0,1"/>
   <wangtile tileid="98" wangid="0,1,0,1,0,0,0,0"/>
   <wangtile tileid="99" wangid="0,0,0,0,0,1,0,1"/>
   <wangtile tileid="128" wangid="0,1,0,1,0,1,0,1"/>
  </wangset>
 </wangsets>
</tileset>
