package com.example.obd2pti.ui.home

import android.graphics.Color
import android.os.Build
import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.TextView
import androidx.annotation.RequiresApi
import androidx.fragment.app.Fragment
import androidx.lifecycle.ViewModelProvider
import com.example.obd2pti.MainActivity
import com.example.obd2pti.databinding.FragmentHomeBinding

class HomeFragment : Fragment() {

    private  lateinit var mainAct: MainActivity
    private var _binding: FragmentHomeBinding? = null

    // This property is only valid between onCreateView and
    // onDestroyView.
    private val binding get() = _binding!!

    @RequiresApi(Build.VERSION_CODES.O)
    override fun onCreateView(
        inflater: LayoutInflater,
        container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View {
        val homeViewModel =
            ViewModelProvider(this).get(HomeViewModel::class.java)
        mainAct = activity as MainActivity

        _binding = FragmentHomeBinding.inflate(inflater, container, false)
        val root: View = binding.root

        val textViewConnection: TextView = binding.textConnection
        homeViewModel.text.observe(viewLifecycleOwner) {
            if (mainAct.connected) {
                textViewConnection.text = "Conectado al dispositivo"
                textViewConnection.setTextColor(Color.GREEN)
            } else {
                textViewConnection.text = "Not Connected"
                textViewConnection.setTextColor(Color.RED)
            }
        }

        val textViewRecoleccion: TextView = binding.textRecoleccion
        homeViewModel.text.observe(viewLifecycleOwner) {
            if (mainAct.thread.recoleccion) {
                textViewRecoleccion.text = "Recolección: ON"
                textViewRecoleccion.setTextColor(Color.GREEN)
            } else {
                textViewRecoleccion.text = "Recolección: OFF"
                textViewRecoleccion.setTextColor(Color.RED)
            }
        }

        binding.recoleccionButton.setOnClickListener {
            if (mainAct.thread.recoleccion) {
                mainAct.thread.recoleccion = false
                textViewRecoleccion.text = "Recolección: OFF"
                textViewRecoleccion.setTextColor(Color.RED)
            }
        }

        binding.uploadButton.setOnClickListener {
            mainAct.uploadData()
        }

        return root
    }

    override fun onDestroyView() {
        super.onDestroyView()
        _binding = null
    }
}