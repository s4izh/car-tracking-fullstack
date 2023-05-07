package com.example.obd2pti

class Datos {
   var matricula : String = ""
   var currentTime : String = ""
   var speed: Int = 0
   var rpm: Int = 0
   var throttlePosition : Float = 0.0f
   var engineLoad :Float = 0.0f
   var coolantTemp : Float = 0.0f
   var oilTemp: Float = 0.0f
   var fuelLevel: Float = 0.0f
   var fuelConsumption : Float = 0.0f


   public fun reset(){
       matricula = ""
       currentTime = ""
       speed = 0
       rpm = 0
       throttlePosition = 0.0f
       engineLoad = 0.0f
       coolantTemp = 0.0f
       oilTemp = 0.0f
       fuelLevel = 0.0f
       fuelConsumption = 0.0f

   }

}