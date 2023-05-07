package com.example.obd2pti

import android.Manifest
import android.annotation.SuppressLint
import android.app.Activity
import android.app.AlertDialog
import android.bluetooth.*
import android.bluetooth.le.*
import android.content.*
import android.content.ContentValues.TAG
import android.content.pm.PackageManager
import android.net.ConnectivityManager
import android.os.*
import android.util.Log
import android.widget.Toast
import androidx.activity.result.contract.ActivityResultContracts
import androidx.annotation.RequiresApi
import androidx.appcompat.app.AppCompatActivity
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import androidx.navigation.findNavController
import androidx.navigation.ui.AppBarConfiguration
import androidx.navigation.ui.setupActionBarWithNavController
import androidx.navigation.ui.setupWithNavController
import com.example.obd2pti.databinding.ActivityMainBinding
import com.github.kittinunf.fuel.httpPost
import com.github.kittinunf.result.*
import com.github.pires.obd.*
import com.google.android.material.bottomnavigation.BottomNavigationView
import java.io.*
import java.lang.reflect.Method
import java.time.LocalDate
import java.util.*

const val REQUEST_ENABLE_BT = 1;
val bluetoothAdapter: BluetoothAdapter? = BluetoothAdapter.getDefaultAdapter()
private const val ENABLE_BLUETOOTH_REQUEST_CODE = 1
private const val LOCATION_PERMISSION_REQUEST_CODE = 2





 class MainActivity : AppCompatActivity() {
     val thread = OBD2Recoletion()
     var recoleccion: Boolean = false
     private val DevicesNames = ArrayList<String>()
    val PairedDevices:MutableMap<String,BluetoothDevice> = mutableMapOf<String, BluetoothDevice>()
    val DiscoveredDevices:MutableMap<String,BluetoothDevice> = mutableMapOf<String, BluetoothDevice>()
    private val DiscoveredDevicesNames = ArrayList<String>()
     private val MY_UUID = UUID.fromString("00001101-0000-1000-8000-00805F9B34FB")
     var workingPath:File = File("")
     var onlyWiFi = false


     //abstract var inputStream:InputStream
   // abstract var outputStream:OutputStream
    var connected = false
    var recolection = false




    fun Context.hasPermission(permissionType: String): Boolean {
        return ContextCompat.checkSelfPermission(this, permissionType) ==
                PackageManager.PERMISSION_GRANTED
    }
    private var requestBluetooth = registerForActivityResult(ActivityResultContracts.StartActivityForResult()) { result ->
        if (result.resultCode == RESULT_OK) {
            //granted
        }else{
            //deny
        }
    }

    private val requestMultiplePermissions =
        registerForActivityResult(ActivityResultContracts.RequestMultiplePermissions()) { permissions ->
            permissions.entries.forEach {
                Log.d("test006", "${it.key} = ${it.value}")
            }
        }


    private lateinit var binding: ActivityMainBinding


    private fun Activity.requestPermission(permission: String, requestCode: Int) {
        ActivityCompat.requestPermissions(this, arrayOf(permission), requestCode)
    }

    private fun requestLocationPermission() {
        val dialogBuilder = AlertDialog.Builder(this)
        dialogBuilder.setTitle("Enable access to Location")
        dialogBuilder.setMessage("Starting from Android M (6.0), the system requires apps to be granted " +
                "location access in order to scan for BLE devices.")
            // if the dialog is cancelable
            .setCancelable(false)
            // positive button text and action
            .setPositiveButton("Enable", DialogInterface.OnClickListener {
                    dialog, id -> dialog.cancel()
                requestPermission(
                    Manifest.permission.ACCESS_FINE_LOCATION,
                    LOCATION_PERMISSION_REQUEST_CODE
                )
                requestPermission(
                    Manifest.permission.ACCESS_FINE_LOCATION,
                    ENABLE_BLUETOOTH_REQUEST_CODE
                )
            })
            // negative button text and action
            .setNegativeButton("Cancel", DialogInterface.OnClickListener {
                    dialog, id -> dialog.cancel()
            })

        // create dialog box
        val alert = dialogBuilder.create()
        // set title for alert dialog box
        alert.setTitle("AlertDialogExample")
        // show alert dialog
        alert.show()
    }

    private val receiver = object : BroadcastReceiver() {

        @SuppressLint("MissingPermission")
        override fun onReceive(context: Context, intent: Intent) {
            val action: String? = intent.action
            when(action) {
                BluetoothDevice.ACTION_FOUND -> {
                    // Discovery has found a device. Get the BluetoothDevice
                    // object and its info from the Intent.
                    val device: BluetoothDevice? = intent.getParcelableExtra(BluetoothDevice.EXTRA_DEVICE)
                    val deviceName = device?.name
                    val deviceHardwareAddress = device?.address // MAC address
                    if (deviceName != null) {
                        DiscoveredDevicesNames.add(deviceName)
                        DiscoveredDevices[deviceName] = device
                    }
                }
            }
        }
    }

    override fun onDestroy() {
        super.onDestroy()
       try {
           stopDiscovery()
       } finally {

       }
        // Don't forget to unregister the ACTION_FOUND receiver.
        unregisterReceiver(receiver)
    }


    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)


       val navView: BottomNavigationView = findViewById(R.id.nav_view)

        val navController = findNavController(R.id.nav_host_fragment_activity_main)
        // Passing each menu ID as a set of Ids because each
        // menu should be considered as top level destinations.
        val appBarConfiguration = AppBarConfiguration(
            setOf(
                R.id.navigation_home, R.id.navigation_dashboard, R.id.navigation_notifications
            )
        )
        setupActionBarWithNavController(navController, appBarConfiguration)
        navView.setupWithNavController(navController)
        workingPath = filesDir
        startDiscovery()
        //testjson(path)

        val wifiFile = File(workingPath, "wifi.txt")
        try {
            onlyWiFi = wifiFile.readText() == "true"
        } catch (e:Exception) {
            wifiFile.createNewFile()
            if (onlyWiFi) {
                wifiFile.writeText("true")
            } else {
                wifiFile.writeText("false")
            }
        }


    }



        public fun getPairedDevices(): ArrayList<String> {
            val isLocationPermissionGranted = hasPermission(Manifest.permission.ACCESS_FINE_LOCATION)
            if (bluetoothAdapter?.isEnabled == false) {
                val enableBtIntent = Intent(BluetoothAdapter.ACTION_REQUEST_ENABLE)
                if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.M && !isLocationPermissionGranted) {
                    requestLocationPermission()
                } else {
                    if (ActivityCompat.checkSelfPermission(
                            this,
                            Manifest.permission.BLUETOOTH_SCAN
                        ) != PackageManager.PERMISSION_GRANTED
                    ) {

                        val myNewVal = ActivityCompat.checkSelfPermission(
                            this,
                            Manifest.permission.BLUETOOTH_SCAN
                        )
                        val myManifest = Manifest.permission.BLUETOOTH_SCAN
                        Log.d("console", "before crash")
                        Log.d("console", "Manifest.permission.BLUETOOTH_SCAN: $myManifest")
                        Log.d("console", "My val: $myNewVal")
                        Log.d(
                            "console",
                            "PackageManager.PERMISSION_GRANTED: ${PackageManager.PERMISSION_GRANTED}"
                        )
                        Log.d(
                            "console",
                            "PackageManager.PERMISSION_GRANTED: ${Manifest.permission.BLUETOOTH_SCAN}"
                        )


                        return DevicesNames
                    }
                    startActivityForResult(enableBtIntent, REQUEST_ENABLE_BT)
                }
            }

            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
                requestMultiplePermissions.launch(arrayOf(
                    Manifest.permission.BLUETOOTH_SCAN,
                    Manifest.permission.BLUETOOTH_CONNECT))
            }
            else{
                val enableBtIntent = Intent(BluetoothAdapter.ACTION_REQUEST_ENABLE)
                requestBluetooth.launch(enableBtIntent)
            }

            val pairedDevices: Set<BluetoothDevice>? = bluetoothAdapter?.bondedDevices

            pairedDevices?.forEach { device ->
                val deviceName = device.name
                val deviceHardwareAddress = device.address // MAC address
                //DevicesNames.add(deviceName)
                PairedDevices[deviceName] = device
                DevicesNames.add(deviceName)
            }

            return DevicesNames
        }

        public fun getDiscoveredDevices(): ArrayList<String> {
            return DiscoveredDevicesNames;
        }

     fun startDiscovery() {
         val isLocationPermissionGranted = hasPermission(Manifest.permission.ACCESS_FINE_LOCATION)
         if (bluetoothAdapter?.isEnabled == false) {
             val enableBtIntent = Intent(BluetoothAdapter.ACTION_REQUEST_ENABLE)
             if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.M && !isLocationPermissionGranted) {
                 requestLocationPermission()
             } else {
                 if (ActivityCompat.checkSelfPermission(
                         this,
                         Manifest.permission.BLUETOOTH_SCAN
                     ) != PackageManager.PERMISSION_GRANTED
                 ) {

                     val myNewVal = ActivityCompat.checkSelfPermission(
                         this,
                         Manifest.permission.BLUETOOTH_SCAN
                     )
                     val myManifest = Manifest.permission.BLUETOOTH_SCAN
                     Log.d("console", "before crash")
                     Log.d("console", "Manifest.permission.BLUETOOTH_SCAN: $myManifest")
                     Log.d("console", "My val: $myNewVal")
                     Log.d(
                         "console",
                         "PackageManager.PERMISSION_GRANTED: ${PackageManager.PERMISSION_GRANTED}"
                     )
                     Log.d(
                         "console",
                         "PackageManager.PERMISSION_GRANTED: ${Manifest.permission.BLUETOOTH_SCAN}"
                     )


                     return
                 }
                 startActivityForResult(enableBtIntent, REQUEST_ENABLE_BT)
             }
         }

         if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
             requestMultiplePermissions.launch(arrayOf(
                 Manifest.permission.BLUETOOTH_SCAN,
                 Manifest.permission.BLUETOOTH_CONNECT))
         }
         else{
             val enableBtIntent = Intent(BluetoothAdapter.ACTION_REQUEST_ENABLE)
             requestBluetooth.launch(enableBtIntent)
         }
         val filter = IntentFilter(BluetoothDevice.ACTION_FOUND)
         registerReceiver(receiver, filter)

         //Start discovery
         bluetoothAdapter?.startDiscovery()
         return
     }

     public fun stopDiscovery() {
         val isLocationPermissionGranted = hasPermission(Manifest.permission.ACCESS_FINE_LOCATION)
         if (bluetoothAdapter?.isEnabled == false) {
             val enableBtIntent = Intent(BluetoothAdapter.ACTION_REQUEST_ENABLE)
             if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.M && !isLocationPermissionGranted) {
                 requestLocationPermission()
             } else {
                 if (ActivityCompat.checkSelfPermission(
                         this,
                         Manifest.permission.BLUETOOTH_SCAN
                     ) != PackageManager.PERMISSION_GRANTED
                 ) {

                     val myNewVal = ActivityCompat.checkSelfPermission(
                         this,
                         Manifest.permission.BLUETOOTH_SCAN
                     )
                     val myManifest = Manifest.permission.BLUETOOTH_SCAN
                     Log.d("console", "before crash")
                     Log.d("console", "Manifest.permission.BLUETOOTH_SCAN: $myManifest")
                     Log.d("console", "My val: $myNewVal")
                     Log.d(
                         "console",
                         "PackageManager.PERMISSION_GRANTED: ${PackageManager.PERMISSION_GRANTED}"
                     )
                     Log.d(
                         "console",
                         "PackageManager.PERMISSION_GRANTED: ${Manifest.permission.BLUETOOTH_SCAN}"
                     )


                     return
                 }
                 startActivityForResult(enableBtIntent, REQUEST_ENABLE_BT)
             }
         }

         if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
             requestMultiplePermissions.launch(arrayOf(
                 Manifest.permission.BLUETOOTH_SCAN,
                 Manifest.permission.BLUETOOTH_CONNECT))
         }
         else{
             val enableBtIntent = Intent(BluetoothAdapter.ACTION_REQUEST_ENABLE)
             requestBluetooth.launch(enableBtIntent)
         }
         val filter = IntentFilter(BluetoothDevice.ACTION_FOUND)
         registerReceiver(receiver, filter)

         //Start discovery
         bluetoothAdapter?.cancelDiscovery()
         return
        }

        @RequiresApi(Build.VERSION_CODES.O)
        public fun connectToBLDevice(device_name:String): Int {
            val device:BluetoothDevice
            connected = false
            stopDiscovery()
            if (PairedDevices.containsKey(device_name)) {
                device = PairedDevices[device_name]!!
            }
            else if (DiscoveredDevices.containsKey(device_name)) {
                device = DiscoveredDevices[device_name]!!
            }
            else {
                return 0
            }
            val isLocationPermissionGranted = hasPermission(Manifest.permission.ACCESS_FINE_LOCATION)
            if (bluetoothAdapter?.isEnabled == false) {
                val enableBtIntent = Intent(BluetoothAdapter.ACTION_REQUEST_ENABLE)
                if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.M && !isLocationPermissionGranted) {
                    requestLocationPermission()
                } else {
                    if (ActivityCompat.checkSelfPermission(
                            this,
                            Manifest.permission.BLUETOOTH_SCAN
                        ) != PackageManager.PERMISSION_GRANTED
                    ) {

                        val myNewVal = ActivityCompat.checkSelfPermission(
                            this,
                            Manifest.permission.BLUETOOTH_SCAN
                        )
                        val myManifest = Manifest.permission.BLUETOOTH_SCAN
                        Log.d("console", "before crash")
                        Log.d("console", "Manifest.permission.BLUETOOTH_SCAN: $myManifest")
                        Log.d("console", "My val: $myNewVal")
                        Log.d(
                            "console",
                            "PackageManager.PERMISSION_GRANTED: ${PackageManager.PERMISSION_GRANTED}"
                        )
                        Log.d(
                            "console",
                            "PackageManager.PERMISSION_GRANTED: ${Manifest.permission.BLUETOOTH_SCAN}"
                        )


                        return 0
                    }
                    startActivityForResult(enableBtIntent, REQUEST_ENABLE_BT)
                }
            }

            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
                requestMultiplePermissions.launch(arrayOf(
                    Manifest.permission.BLUETOOTH_SCAN,
                    Manifest.permission.BLUETOOTH_CONNECT))
            }
            else{
                val enableBtIntent = Intent(BluetoothAdapter.ACTION_REQUEST_ENABLE)
                requestBluetooth.launch(enableBtIntent)
            }
            val filter = IntentFilter(BluetoothDevice.ACTION_FOUND)
            registerReceiver(receiver, filter)
            Toast.makeText(this, "Conectando a ${device.name}", Toast.LENGTH_SHORT).show()

            var sock: BluetoothSocket? = null
            var sockFallback: BluetoothSocket? = null

            Log.d(TAG, "Starting Bluetooth connection..")
            try {
                sock = device.createRfcommSocketToServiceRecord(MY_UUID)
                sock.connect()
                connected = true
                obd2Connection(workingPath, sock)
                return 1
            } catch (e1: Exception) {
                Log.e(
                    TAG,
                    "There was an error while establishing Bluetooth connection. Falling back..",
                    e1
                )
                val clazz: Class<*> = sock!!.remoteDevice.javaClass
                val paramTypes = arrayOf<Class<*>>(Integer.TYPE)
                try {
                    val m: Method = clazz.getMethod("createRfcommSocket", *paramTypes)
                    val params = arrayOf<Any>(Integer.valueOf(1))
                    sockFallback = m.invoke(sock!!.remoteDevice, params) as BluetoothSocket?
                    sockFallback!!.connect()
                    sock = sockFallback
                } catch (e2: Exception) {
                    Log.e(TAG, "Couldn't fallback while establishing Bluetooth connection.", e2)
                    Toast.makeText(this, "No se pudo conectar a ${device.name}", Toast.LENGTH_SHORT).show()
                    connected = false
                    return 0
                  //  throw IOException(e2.message)
                }
            }
                return 0

        }



         @RequiresApi(Build.VERSION_CODES.O)
         fun obd2Connection(path: File, socket: BluetoothSocket) {
            if (!connected) {
                Toast.makeText(this@MainActivity, "No hay conexión", Toast.LENGTH_SHORT).show()
                return
            }

            val letDirectory = File(path, "EXPORTS")
            if (!letDirectory.exists()) {
                letDirectory.mkdir()
            }
             //Get a string with the current date
            val today = LocalDate.now().toString()
            val file = File(letDirectory, "${today}.json")
            if (!file.exists()) {
                file.createNewFile()
            }
             thread.socket = socket
             thread.path = workingPath

             thread.start()
            return
        }

     fun uploadData() {
         /*val httpAsync = "https://httpbin.org/get"
             .httpGet()
             .responseString { request, response, result ->
                 when (result) {
                     is Result.Failure -> {
                         val ex = result.getException()
                         println(ex)
                         Toast.makeText(this, "Error al subir datos", Toast.LENGTH_SHORT).show()
                         Toast.makeText(this, ex.toString(), Toast.LENGTH_SHORT).show()
                     }
                     is Result.Success -> {
                         val data = result.get()
                         println(data)
                        Toast.makeText(this, "Datos subidos", Toast.LENGTH_SHORT).show()
                         Toast.makeText(this, data, Toast.LENGTH_SHORT).show()
                     }
                 }
             }*/

         //httpAsync.join()
         //check if there is a wifi connection
         var ficherosSubidos = ArrayList<File>()
         val connectivityManager =
             getSystemService(Context.CONNECTIVITY_SERVICE) as ConnectivityManager
         val activeNetworkInfo = connectivityManager.activeNetworkInfo
         val exportDirectory = File(workingPath, "EXPORTS")
         if (onlyWiFi && activeNetworkInfo != null && activeNetworkInfo.type == ConnectivityManager.TYPE_WIFI) {
             Toast.makeText(this, "Conectado a wifi", Toast.LENGTH_SHORT).show()
             exportDirectory.walk().forEach {
                 if (it.isFile) {
                     val httpPostAsync = "https://httpbin.org/post"
                         .httpPost()
                         .body(it.readText())
                         .responseString { request, response, result ->
                             when (result) {
                                 is Result.Failure -> {
                                     val ex = result.getException()
                                     println(ex)
                                     Toast.makeText(
                                         this,
                                         "Error al subir datos",
                                         Toast.LENGTH_SHORT
                                     ).show()
                                     Toast.makeText(this, ex.toString(), Toast.LENGTH_SHORT)
                                         .show()
                                 }

                                 is Result.Success -> {
                                     val data = result.get()
                                     println(data)
                                     Toast.makeText(this, "Datos subidos ${it.name}", Toast.LENGTH_SHORT)
                                         .show()
                                     Toast.makeText(this, data, Toast.LENGTH_SHORT).show()
                                     ficherosSubidos.add(it)
                                 }
                             }
                         }
                     httpPostAsync.join()
                 }
             }
         } else if (!onlyWiFi){
                Toast.makeText(this, "No hay conexión a wifi", Toast.LENGTH_SHORT).show()
             exportDirectory.walk().forEach {
                 if (it.isFile) {
                     val httpPostAsync = "https://httpbin.org/post"
                         .httpPost()
                         .body(it.readText())
                         .responseString { request, response, result ->
                             when (result) {
                                 is Result.Failure -> {
                                     val ex = result.getException()
                                     println(ex)
                                     Toast.makeText(
                                         this,
                                         "Error al subir datos",
                                         Toast.LENGTH_SHORT
                                     ).show()
                                     Toast.makeText(this, ex.toString(), Toast.LENGTH_SHORT).show()
                                 }

                                 is Result.Success -> {
                                     val data = result.get()
                                     println(data)
                                     Toast.makeText(this, "Datos subidos de ${it.name}", Toast.LENGTH_SHORT)
                                         .show()
                                     Toast.makeText(this, data, Toast.LENGTH_SHORT).show()
                                     ficherosSubidos.add(it)
                                 }
                             }
                         }
                     httpPostAsync.join()
                 }
             }
     }
         ficherosSubidos.forEach(){
             //it.delete()
             Toast.makeText(this, "Borrando ${it.name}", Toast.LENGTH_SHORT).show()
         }


     }

 }
