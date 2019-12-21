package io.marcopolo.pietdemo

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle

class MainActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        System.loadLibrary("pietdemo")
        setContentView(R.layout.activity_main)
        initRust()
    }

    external fun initRust()
}
