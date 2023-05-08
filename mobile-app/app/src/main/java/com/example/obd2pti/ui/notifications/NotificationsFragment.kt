package com.example.obd2pti.ui.notifications

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.Button
import android.widget.EditText
import android.widget.Toast
import androidx.fragment.app.Fragment
import androidx.lifecycle.ViewModelProvider
import com.example.obd2pti.MainActivity
import com.example.obd2pti.databinding.FragmentNotificationsBinding
import java.io.File
import java.math.BigInteger
import java.security.MessageDigest

class NotificationsFragment : Fragment() {

    private var _binding: FragmentNotificationsBinding? = null

    // This property is only valid between onCreateView and
    // onDestroyView.
    private val binding get() = _binding!!

    override fun onCreateView(
        inflater: LayoutInflater,
        container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View {
        val notificationsViewModel =
            ViewModelProvider(this).get(NotificationsViewModel::class.java)

        _binding = FragmentNotificationsBinding.inflate(inflater, container, false)
        val root: View = binding.root

        var mainAct = activity as MainActivity

        val editTextMatricula: EditText = binding.editTextMatricula
        val editTextPassword: EditText = binding.editTextPassword
        val loginButton: Button = binding.loginbutton
        val checkBoxWiFi = binding.checkBoxWifi

        var matricula: String
        var password: String


        val wifiFile = File(mainAct.workingPath, "wifi.txt")
        try {
            checkBoxWiFi.isChecked = wifiFile.readText() == "true"
        } catch (e:Exception) {
            wifiFile.createNewFile()
            checkBoxWiFi.isChecked = false
        }



        loginButton.setOnClickListener{
            matricula = editTextMatricula.text.toString()
            password = editTextPassword.text.toString()
            var md = MessageDigest.getInstance("SHA-256")
            val passwordHash = BigInteger(1, md.digest(password.toByteArray())).toString(16).padStart(32, '0')
            //Get path to app direcotry
            val path = mainAct.workingPath
            val file = File(path, "matricula.txt")
            if (!file.exists()) {
                file.createNewFile()
            }
            //Write text to file
            file.writeText("");
            file.writeText(matricula)
            val file2 = File(path, "password.txt")
            if (!file2.exists()) {
                file2.createNewFile()
            }
            //Write text to file
            file2.writeText("");
            //file2.writeText(passwordHash)
            file2.writeText(password)
            Toast.makeText(context, "Usuario Registrado", Toast.LENGTH_SHORT).show()
        }
        checkBoxWiFi.setOnClickListener{
            mainAct.onlyWiFi = checkBoxWiFi.isChecked
            try {
                if(checkBoxWiFi.isChecked) {
                    wifiFile.writeText("true")
                } else {
                    wifiFile.writeText("false")
                }
            } catch (e:Exception) {
                wifiFile.createNewFile()
                if(checkBoxWiFi.isChecked) {
                    wifiFile.writeText("true")
                } else {
                    wifiFile.writeText("false")
                }
            }
        }

        return root
    }

    override fun onDestroyView() {
        super.onDestroyView()
        _binding = null
    }
}