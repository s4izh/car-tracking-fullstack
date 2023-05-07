package com.example.obd2pti
import android.bluetooth.BluetoothSocket
import android.os.Build
import android.util.JsonWriter
import androidx.annotation.RequiresApi
import com.github.pires.obd.*
import com.github.pires.obd.commands.SpeedCommand
import com.github.pires.obd.commands.control.TroubleCodesCommand
import com.github.pires.obd.commands.engine.LoadCommand
import com.github.pires.obd.commands.engine.OilTempCommand
import com.github.pires.obd.commands.engine.RPMCommand
import com.github.pires.obd.commands.engine.ThrottlePositionCommand
import com.github.pires.obd.commands.fuel.ConsumptionRateCommand
import com.github.pires.obd.commands.fuel.FuelLevelCommand
import com.github.pires.obd.commands.protocol.EchoOffCommand
import com.github.pires.obd.commands.protocol.LineFeedOffCommand
import com.github.pires.obd.commands.protocol.SelectProtocolCommand
import com.github.pires.obd.commands.protocol.TimeoutCommand
import com.github.pires.obd.commands.temperature.AmbientAirTemperatureCommand
import com.github.pires.obd.commands.temperature.EngineCoolantTemperatureCommand
import java.io.File
import java.io.FileWriter
import java.time.LocalDateTime

class OBD2Recoletion(): Thread() {

    //private var recolección = false
    lateinit var socket: BluetoothSocket
    var recoleccion = false
    lateinit var path:File

    @RequiresApi(Build.VERSION_CODES.O)
    override fun run() {
        var matricula : String
        var passwordHash: String
        val matriculaFile = File(path, "matricula.txt")
       try {
              matricula = matriculaFile.readText()
         } catch (e:Exception) {
              matriculaFile.createNewFile()
              matriculaFile.writeText("matricula")
              matricula = matriculaFile.readText()
       }
        val hashFile = File(path, "password.txt")
        try {
            passwordHash = hashFile.readText()
        } catch (e:Exception) {
            hashFile.createNewFile()
            hashFile.writeText("nullHash")
            passwordHash = matriculaFile.readText()
        }
        val letDirectory = File(path, "EXPORTS")
        if (!letDirectory.exists()) {
            letDirectory.mkdir()
        }
        //Get a string with the current date
        val time = LocalDateTime.now()
        val file = File(letDirectory, "${time}.json")
        if (!file.exists()) {
            file.createNewFile()
        }
        //delete file content
       // file.writeText("")
        val fileWriter = FileWriter(file, true)
        val jsonWriter = JsonWriter(fileWriter)


        val listaDatos: MutableList<Datos> = mutableListOf()

        try {
            EchoOffCommand().run(socket.inputStream, socket.outputStream)
            LineFeedOffCommand().run(socket.inputStream, socket.outputStream)
            TimeoutCommand(500).run(socket.inputStream, socket.outputStream)
            SelectProtocolCommand(com.github.pires.obd.enums.ObdProtocols.AUTO).run(socket.inputStream, socket.outputStream)
            AmbientAirTemperatureCommand().run(
                socket.inputStream,
                socket.outputStream
            )
            recoleccion = true



        } catch (e: java.lang.Exception) {
            // handle errors
        }
        var speedcomm:SpeedCommand = SpeedCommand()
        var rpmcomm:RPMCommand = RPMCommand()
        var throttlecomm:ThrottlePositionCommand = ThrottlePositionCommand()
        var engineloadcomm:LoadCommand = LoadCommand()
        var coolanttempcomm:EngineCoolantTemperatureCommand = EngineCoolantTemperatureCommand()
        var oiltempcomm:OilTempCommand = OilTempCommand()
        var fuellevelcomm:FuelLevelCommand = FuelLevelCommand()
        var fuelconsumptioncomm: ConsumptionRateCommand = ConsumptionRateCommand()
        var troublecodescomm: TroubleCodesCommand = TroubleCodesCommand()


        var velocidades: MutableList<Int> = mutableListOf()

        var queryNum = 0
        var lastTime = System.currentTimeMillis()
        var firstTime = System.currentTimeMillis()
        var first_fuel:Float = 0.0f
        var last_fuel:Float = 0.0f
        var troubleCodes:String = ""
        recoleccion = true
        while (recoleccion) {
            // Toast.makeText(this, "Query: $queryNum", Toast.LENGTH_SHORT).show()
            var datos = Datos()
           try {
               rpmcomm.run(socket.inputStream, socket.outputStream)
           } catch (e: Exception) {
              // println("Error al leer RPM");
              }
            try {
                speedcomm.run(socket.inputStream, socket.outputStream)
            } catch (e: Exception) {
                //println("Error al leer velocidad");
            }
            try {jsonWriter.endObject()
                throttlecomm.run(socket.inputStream, socket.outputStream)
            } catch (e: Exception) {
                //println("Error al leer throttle");
            }
            try {
                engineloadcomm.run(socket.inputStream, socket.outputStream)
            } catch (e: Exception) {
                //println("Error al leer engine load");
            }
            try {
                coolanttempcomm.run(socket.inputStream, socket.outputStream)
            } catch (e: Exception) {
                //println("Error al leer coolant temp");
            }
            try {
                oiltempcomm.run(socket.inputStream, socket.outputStream)
            } catch (e: Exception) {
                //println("Error al leer oil temp");
            }
            try {
                fuellevelcomm.run(socket.inputStream, socket.outputStream)
            } catch (e: Exception) {
                //println("Error al leer fuel level");
            }
            try {
                fuelconsumptioncomm.run(socket.inputStream, socket.outputStream)
            } catch (e: Exception) {
                //println("Error al leer fuel consumption");
            }
            if (queryNum == 1 )try {
                troublecodescomm.run(socket.inputStream, socket.outputStream)
            } catch (e: Exception) {
                //println("Error al leer trouble codes");
            }
            val current = LocalDateTime.now()
            datos.matricula = matricula
            datos.currentTime = current.toString()
            datos.speed = speedcomm.metricSpeed
            velocidades.add(datos.speed)
            datos.rpm = rpmcomm.rpm
            datos.throttlePosition = throttlecomm.percentage
            datos.engineLoad = engineloadcomm.percentage
            datos.coolantTemp = coolanttempcomm.temperature
            datos.oilTemp = oiltempcomm.temperature
            datos.fuelLevel = fuellevelcomm.fuelLevel
            datos.fuelConsumption = fuelconsumptioncomm.litersPerHour
            listaDatos.add(datos)
            if (queryNum == 1) {
                first_fuel = datos.fuelLevel
                troubleCodes = troublecodescomm.formattedResult
            }
           lastTime = System.currentTimeMillis()
            last_fuel = datos.fuelLevel
            ++queryNum
            Thread.sleep(750)
        }
        val avgSpeed = velocidades.average();
        val duration = (lastTime - firstTime) / 1000
        val km = avgSpeed * (duration.toDouble() / 3600)
        val consumed_fuel = first_fuel - last_fuel
        jsonWriter.beginObject()
        jsonWriter.name("matricula").value(matricula)
        jsonWriter.name("hash").value(passwordHash)
        jsonWriter.name("km").value(km.toInt())
        jsonWriter.name("max_speed").value(velocidades.max())
        jsonWriter.name("speed_average").value(avgSpeed)
        jsonWriter.name("fuel_percentage").value(consumed_fuel)
        jsonWriter.name("duration").value(duration)
        jsonWriter.name("trouble_codes").value(troubleCodes)
        jsonWriter.name("data").beginArray()
        listaDatos.forEach() {
            jsonWriter.beginObject()
            jsonWriter.name("timestamp").value(it.currentTime)
            jsonWriter.name("speed").value(it.speed)
            jsonWriter.name("rpm").value(it.rpm)
            jsonWriter.name("throttle").value(it.throttlePosition)
            jsonWriter.name("engine_load").value(it.engineLoad)
            jsonWriter.name("engine_coolant_temp").value(it.coolantTemp)
            jsonWriter.name("oil_temp").value(it.oilTemp)
            jsonWriter.name("fuel_level").value(it.fuelLevel)
            jsonWriter.name("fuel_consumption").value(it.fuelConsumption)
            jsonWriter.endObject()
        }
        jsonWriter.endArray()
        jsonWriter.endObject()
        jsonWriter.close()
        fileWriter.close()
        return
    }

}
